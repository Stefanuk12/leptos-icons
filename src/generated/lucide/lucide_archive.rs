use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "1" x = "2" width = "20" height = "5" ></ rect > < path d = "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" ></ path > < path d = "M10 12h4" ></ path > < / > } } pub const LUCIDE_ARCHIVE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;