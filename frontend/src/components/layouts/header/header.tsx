import Link from "next/link";
import { ModeToggle } from "./mode-toggle";
const Header = () => {
  return (
    <header className="flex border">
      <nav className="flex">
        <Link href={"/"} className="p-1">
          home
        </Link>
        <Link href={"/add"} className="p-1">
          add
        </Link>
        <Link href={"/record"} className="p-1">
          record
        </Link>
        <Link href={"/quiz"} className="p-1">
          quiz
        </Link>
      </nav>
      <ModeToggle />
    </header>
  );
};
export default Header;
