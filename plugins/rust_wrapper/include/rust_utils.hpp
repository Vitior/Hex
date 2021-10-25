#pragma once

#include <cxx.h>

namespace hex { struct View; }

namespace hex::rust {

    using ViewRustSimpleFunc = ::rust::Fn<void()>;
    using ViewRustBoolFunc = ::rust::Fn<bool()>;
    using ViewRustContentDrawFunc = ::rust::Fn<void(bool&)>;
    using ViewRustShortcutFunc = ::rust::Fn<bool(bool[512], bool, bool, bool)>;

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
                         ViewRustBoolFunc hasViewMenuItemEntryFunc);
}