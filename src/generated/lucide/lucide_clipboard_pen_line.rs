use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "8" x = "8" height = "4" y = "2" ></ rect > < path d = "M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5" ></ path > < path d = "M16 4h2a2 2 0 0 1 1.73 1" ></ path > < path d = "M8 18h1" ></ path > < path d = "M18.4 9.6a2 2 0 0 1 3 3L17 17l-4 1 1-4Z" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_PEN_LINE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;