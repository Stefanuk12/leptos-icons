use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5" ></ path > < path d = "M16 4h2a2 2 0 0 1 1.73 1" ></ path > < path d = "M8 18h1" ></ path > < path d = "M18.4 9.6a2 2 0 0 1 3 3L17 17l-4 1 1-4Z" ></ path > < / > } } pub const LucideClipboardPenLine : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;