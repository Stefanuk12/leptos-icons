use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" height = "5" y = "3" x = "2" width = "20" ></ rect > < path d = "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" ></ path > < path d = "M10 12h4" ></ path > < / > } } pub const LUCIDE_ARCHIVE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;