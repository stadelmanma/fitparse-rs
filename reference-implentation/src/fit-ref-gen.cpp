////////////////////////////////////////////////////////////////////////////////
// The following FIT Protocol software provided may be used with FIT protocol
// devices only and remains the copyrighted property of Garmin Canada Inc.
// The software is being provided on an "as-is" basis and as an accommodation,
// and therefore all warranties, representations, or guarantees of any kind
// (whether express, implied or statutory) including, without limitation,
// warranties of merchantability, non-infringement, or fitness for a particular
// purpose, are specifically disclaimed.
//
// Copyright 2008 Garmin Canada Inc.
////////////////////////////////////////////////////////////////////////////////

#include <fstream>
#include <iostream>

#include "fit_decode.hpp"
#include "fit_mesg_broadcaster.hpp"
#include "fit_developer_field_description.hpp"

class Listener
    : public fit::MesgListener,
      public fit::DeveloperFieldDescriptionListener
{
public:
    static void PrintValues(const fit::FieldBase &field)
    {
        for (FIT_UINT8 j = 0; j < (FIT_UINT8)field.GetNumValues(); j++)
        {
            std::wcout << L"       Val" << j << L": ";
            switch (field.GetType())
            {
            // Get float 64 values for numeric types to receive values that have
            // their scale and offset properly applied.
            case FIT_BASE_TYPE_ENUM:
            case FIT_BASE_TYPE_BYTE:
            case FIT_BASE_TYPE_SINT8:
            case FIT_BASE_TYPE_UINT8:
            case FIT_BASE_TYPE_SINT16:
            case FIT_BASE_TYPE_UINT16:
            case FIT_BASE_TYPE_SINT32:
            case FIT_BASE_TYPE_UINT32:
            case FIT_BASE_TYPE_SINT64:
            case FIT_BASE_TYPE_UINT64:
            case FIT_BASE_TYPE_UINT8Z:
            case FIT_BASE_TYPE_UINT16Z:
            case FIT_BASE_TYPE_UINT32Z:
            case FIT_BASE_TYPE_UINT64Z:
            case FIT_BASE_TYPE_FLOAT32:
            case FIT_BASE_TYPE_FLOAT64:
                std::wcout << field.GetFLOAT64Value(j);
                break;
            case FIT_BASE_TYPE_STRING:
                std::wcout << field.GetSTRINGValue(j);
                break;
            default:
                break;
            }
            std::wcout << L" " << field.GetUnits().c_str() << L"\n";
            ;
        }
    }

    void OnMesg(fit::Mesg &mesg)
    {
        printf("On Mesg:\n");
        std::wcout << L"   New Mesg: " << mesg.GetName().c_str() << L".  It has " << mesg.GetNumFields() << L" field(s) and " << mesg.GetNumDevFields() << " developer field(s).\n";

        for (FIT_UINT16 i = 0; i < (FIT_UINT16)mesg.GetNumFields(); i++)
        {
            fit::Field *field = mesg.GetFieldByIndex(i);
            std::wcout << L"   Field" << i << " (" << field->GetName().c_str() << ") has " << field->GetNumValues() << L" value(s)\n";
            PrintValues(*field);
        }

        for (auto devField : mesg.GetDeveloperFields())
        {
            std::wcout << L"   Developer Field(" << devField.GetName().c_str() << ") has " << devField.GetNumValues() << L" value(s)\n";
            PrintValues(devField);
        }
    }

    static void PrintOverrideValues(const fit::Mesg &mesg, FIT_UINT8 fieldNum)
    {
        std::vector<const fit::FieldBase *> fields = mesg.GetOverrideFields(fieldNum);
        const fit::Profile::FIELD *profileField = fit::Profile::GetField(mesg.GetNum(), fieldNum);
        FIT_BOOL namePrinted = FIT_FALSE;

        for (const fit::FieldBase *field : fields)
        {
            if (!namePrinted)
            {
                printf("   %s:\n", profileField->name.c_str());
                namePrinted = FIT_TRUE;
            }

            if (FIT_NULL != dynamic_cast<const fit::Field *>(field))
            {
                // Native Field
                printf("      native: ");
            }
            else
            {
                // Developer Field
                printf("      override: ");
            }

            switch (field->GetType())
            {
            // Get float 64 values for numeric types to receive values that have
            // their scale and offset properly applied.
            case FIT_BASE_TYPE_ENUM:
            case FIT_BASE_TYPE_BYTE:
            case FIT_BASE_TYPE_SINT8:
            case FIT_BASE_TYPE_UINT8:
            case FIT_BASE_TYPE_SINT16:
            case FIT_BASE_TYPE_UINT16:
            case FIT_BASE_TYPE_SINT32:
            case FIT_BASE_TYPE_UINT32:
            case FIT_BASE_TYPE_SINT64:
            case FIT_BASE_TYPE_UINT64:
            case FIT_BASE_TYPE_UINT8Z:
            case FIT_BASE_TYPE_UINT16Z:
            case FIT_BASE_TYPE_UINT32Z:
            case FIT_BASE_TYPE_UINT64Z:
            case FIT_BASE_TYPE_FLOAT32:
            case FIT_BASE_TYPE_FLOAT64:
                printf("%f\n", field->GetFLOAT64Value());
                break;
            case FIT_BASE_TYPE_STRING:
                printf("%ls\n", field->GetSTRINGValue().c_str());
                break;
            default:
                break;
            }
        }
    }

    void OnDeveloperFieldDescription(const fit::DeveloperFieldDescription &desc) override
    {
        printf("New Developer Field Description\n");
        printf("   App Version: %d\n", desc.GetApplicationVersion());
        printf("   Field Number: %d\n", desc.GetFieldDefinitionNumber());
    }
};

int main(int argc, char *argv[])
{
    fit::Decode decode;
    // decode.SkipHeader();       // Use on streams with no header and footer (stream contains FIT defn and data messages only)
    // decode.IncompleteStream(); // This suppresses exceptions with unexpected eof (also incorrect crc)
    fit::MesgBroadcaster mesgBroadcaster;
    Listener listener;
    std::fstream file;

    printf("FIT Decode Example Application\n");

    if (argc != 2)
    {
        printf("Usage: decode.exe <filename>\n");
        return -1;
    }

    file.open(argv[1], std::ios::in | std::ios::binary);

    if (!file.is_open())
    {
        printf("Error opening file %s\n", argv[1]);
        return -1;
    }

    if (!decode.CheckIntegrity(file))
    {
        printf("FIT file integrity failed.\nAttempting to decode...\n");
    }

    mesgBroadcaster.AddListener((fit::MesgListener &)listener);

    try
    {
        decode.Read(&file, &mesgBroadcaster, &mesgBroadcaster, &listener);
    }
    catch (const fit::RuntimeException &e)
    {
        printf("Exception decoding file: %s\n", e.what());
        return -1;
    }

    printf("Decoded FIT file %s.\n", argv[1]);

    return 0;
}
