#include "rust_utils.hpp"

#include <views/view_rust.hpp>

namespace hex::rust {

    View* createRustView(const std::string &unlocalizedName,
                         bool destructorValid,
                         ViewRustSimpleFunc destructorFunc,
                         bool drawContentValid,
                         ViewRustContentDrawFunc drawContentFunc,
                         bool drawAlwaysVisibleValid,
                         ViewRustSimpleFunc drawAlwaysVisibleFunc,
                         bool drawMenuValid,
                         ViewRustSimpleFunc drawMenuFunc,
                         bool isAvailableValid,
                         ViewRustBoolFunc isAvailableFunc,
                         bool shouldProcessValid,
                         ViewRustBoolFunc shouldProcessFunc,
                         bool hasViewMenuItemEntryValid,
                         ViewRustBoolFunc hasViewMenuItemEntryFunc) {

        return new rust::ViewRustWrapper(unlocalizedName, {
                destructorValid,
                destructorFunc,
                drawContentValid,
                drawContentFunc,
                drawAlwaysVisibleValid,
                drawAlwaysVisibleFunc,
                drawMenuValid,
                drawMenuFunc,
                isAvailableValid,
                isAvailableFunc,
                shouldProcessValid,
                shouldProcessFunc,
                hasViewMenuItemEntryValid,
                hasViewMenuItemEntryFunc
        });
    }

}