use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "5" y = "3" x = "2" rx = "1" width = "20" ></ rect > < path d = "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" ></ path > < path d = "M10 12h4" ></ path > < / > } } pub const LUCIDE_ARCHIVE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;