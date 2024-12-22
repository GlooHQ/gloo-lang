"use client";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { atom, useAtom, useAtomValue } from "jotai";
import { Braces, Bug, ChevronDown, FileJson, PlayCircle } from "lucide-react";
import React from "react";
import { ThemeToggle } from "../theme/ThemeToggle";
import { areTestsRunningAtom, selectedItemAtom } from "./atoms";
import { FunctionTestName } from "./function-test-name";
import { useRunTests } from "./prompt-preview/test-panel/test-runner";

export const renderModeAtom = atom<"prompt" | "curl" | "tokens">("prompt");

const RunButton: React.FC = () => {
  const { setRunningTests } = useRunTests();
  const isRunning = useAtomValue(areTestsRunningAtom);
  const selected = useAtomValue(selectedItemAtom);
  return (
    <Button
      variant="default"
      size="sm"
      className="h-8 bg-purple-500  text-sm text-white hover:bg-purple-700 disabled:bg-muted disabled:text-muted-foreground dark:bg-purple-700 dark:text-foreground dark:hover:bg-purple-800"
      disabled={isRunning || selected === undefined}
      onClick={() => {
        if (selected) {
          void setRunningTests([
            { functionName: selected[0], testName: selected[1] },
          ]);
        }
      }}
    >
      <PlayCircle className="mr-0 h-4 w-4" />
      Run {selected ? selected[1] : ""}
    </Button>
  );
};

export default function Component() {
  const [renderMode, setRenderMode] = useAtom(renderModeAtom);
  const selections = useAtomValue(selectedItemAtom);

  const options: {
    label: string;
    icon: React.FC<React.SVGProps<SVGSVGElement>>;
    value: "prompt" | "curl" | "tokens";
  }[] = [
    { label: "Prompt", icon: FileJson, value: "prompt" },
    { label: "Token Visualization", icon: Braces, value: "tokens" },
    { label: "Raw cURL", icon: Bug, value: "curl" },
  ];

  const selectedOption = options.find((opt) => opt.value === renderMode);

  const SelectedIcon = selectedOption?.icon || FileJson;

  return (
    <div className="flex flex-col gap-1">
      {selections !== undefined && (
        <div className="flex flex-row gap-1">
          <FunctionTestName
            functionName={selections[0]}
            testName={selections[1]}
          />
          <ThemeToggle />
        </div>
      )}
      <div className="flex w-full items-center space-x-4">
        <RunButton />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              variant="outline"
              size="sm"
              className="h-8 border-border bg-background hover:bg-accent hover:text-accent-foreground"
            >
              <SelectedIcon className="mr-2 h-4 w-4" />
              {selectedOption?.label}
              <ChevronDown className="ml-2 h-4 w-4 opacity-50" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            align="start"
            className="border-border bg-background"
          >
            {options.map((option) => (
              <DropdownMenuItem
                key={option.label}
                onSelect={() => setRenderMode(option.value)}
                className="hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground"
              >
                <option.icon className="mr-2 h-4 w-4" />
                {option.label}
              </DropdownMenuItem>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  );
}
