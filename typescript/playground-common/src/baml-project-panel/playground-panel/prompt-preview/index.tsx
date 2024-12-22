"use client";

import { useEffect, useRef, useState } from "react";
import { ChevronUp } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";
import PreviewToolbar from "../preview-toolbar";
import SideBar from "../side-bar";
import { PromptRenderWrapper } from "./prompt-render-wrapper";
import TestPanel from "./test-panel";
import {
  ResizableHandle,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { type ImperativePanelHandle } from "react-resizable-panels";
import { ResizablePanel } from "@/components/ui/resizable";
import { useAtomValue } from "jotai";
import { areTestsRunningAtom } from "../atoms";
import { ThemeProvider } from "../../theme/ThemeProvider";
const PromptPreview = () => {
  const areTestsRunning = useAtomValue(areTestsRunningAtom);
  const ref = useRef<ImperativePanelHandle>(null);

  const handleResize = () => {
    if (ref.current) {
      if (areTestsRunning) {
        // expand the test panel to 70% of the height
        console.log("ref.current.getSize()", ref.current.getSize());
        if (ref.current.getSize() < 60) {
          console.log("resizing to 70");
          ref.current.resize(80);
        }
      } else {
        // ref.current.resize(20);
      }
    }
  };

  useEffect(() => {
    handleResize();
  }, [areTestsRunning]);

  return (
    <div className="relative flex h-full justify-between bg-background text-foreground">
      <div
        className="flex h-full w-full flex-col items-start justify-start overflow-x-auto"
        style={{ minHeight: "530px" }}
      >
        <ResizablePanelGroup
          autoSaveId={"prompt-preview"}
          direction="vertical"
          className="h-full py-2"
        >
          <ResizablePanel
            defaultSize={areTestsRunning ? 40 : 80}
            className="flex flex-col gap-4 px-4"
          >
            <PreviewToolbar />
            <ScrollArea
              className="h-full w-full rounded-md  bg-background"
              type="always"
            >
              <div className="h-fit w-full rounded-md border border-border/50 bg-background">
                <PromptRenderWrapper />
              </div>
            </ScrollArea>
          </ResizablePanel>
          <ResizableHandle withHandle className="bg-border" />
          <ResizablePanel
            ref={ref}
            defaultSize={areTestsRunning ? 60 : 20}
            className="flex flex-col pl-2"
          >
            <TestPanel />
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
      <SideBar />
    </div>
  );
};

export default PromptPreview;
