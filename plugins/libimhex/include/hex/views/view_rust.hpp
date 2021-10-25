#pragma once

#include <hex.hpp>

#include <hex/views/view.hpp>

#include <imgui.h>

#include <functional>
#include <string>
#include <vector>

namespace hex::rust {

    using ViewRustSimpleFunc = ::rust::Fn<void()>;
    using ViewRustBoolFunc = ::rust::Fn<bool()>;
    using ViewRustContentDrawFunc = ::rust::Fn<void(bool&)>;
    using ViewRustShortcutFunc = ::rust::Fn<bool(bool[512], bool, bool, bool)>;
    using ViewRustVecFunc = ::rust::Fn<ImVec2()>;

    struct ViewRustFunctions {
        ViewRustSimpleFunc destructorFunc;
        ViewRustContentDrawFunc drawContentFunc;
        ViewRustSimpleFunc drawAlwaysVisibleFunc;
        ViewRustSimpleFunc drawMenuFunc;
        ViewRustBoolFunc isAvailableFunc;
        ViewRustBoolFunc shouldProcessFunc;
        ViewRustBoolFunc hasViewMenuItemEntryFunc;
    };

    class ViewRustWrapper : public View {
    public:
        explicit ViewRustWrapper(std::string unlocalizedViewName, ViewRustFunctions funcs) : View(unlocalizedViewName), m_funcs(funcs) {

        }
        ~ViewRustWrapper() override {
            this->m_funcs.destructorFunc();
        }

        void drawContent() override {
            this->m_funcs.drawContentFunc(this->getWindowOpenState());
        }

        void drawAlwaysVisible() override {
            this->m_funcs.drawAlwaysVisibleFunc();
        }

        void drawMenu() override {
            this->m_funcs.drawMenuFunc();
        }

        bool handleShortcut(bool keys[512], bool ctrl, bool shift, bool alt) override {
            return View::handleShortcut(keys, ctrl, shift, alt);
        }

        bool isAvailable() override {
            return this->m_funcs.isAvailableFunc();
        }

        bool shouldProcess() override {
            return this->m_funcs.shouldProcessFunc();
        }

        bool hasViewMenuItemEntry() override {
            return this->m_funcs.hasViewMenuItemEntryFunc();
        }

        ImVec2 getMinSize() override {
            return View::getMinSize();
        }

        ImVec2 getMaxSize() override {
            return View::getMaxSize();
        }

    private:
        ViewRustFunctions m_funcs;
    };

}