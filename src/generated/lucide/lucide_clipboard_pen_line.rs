use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "8" y = "2" width = "8" rx = "1" height = "4" ></ rect > < path d = "M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5" ></ path > < path d = "M16 4h2a2 2 0 0 1 1.73 1" ></ path > < path d = "M8 18h1" ></ path > < path d = "M21.378 12.626a1 1 0 0 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z" ></ path > < / > } } pub const LUCIDE_CLIPBOARD_PEN_LINE : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;