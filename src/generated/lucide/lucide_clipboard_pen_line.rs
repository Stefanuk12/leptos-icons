use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "2" width = "8" height = "4" x = "8" rx = "1" ></ rect > < path d = "M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5" ></ path > < path d = "M16 4h2a2 2 0 0 1 1.73 1" ></ path > < path d = "M8 18h1" ></ path > < path d = "M18.4 9.6a2 2 0 0 1 3 3L17 17l-4 1 1-4Z" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_PEN_LINE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;