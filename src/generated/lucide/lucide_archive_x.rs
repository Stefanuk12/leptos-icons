use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "1" width = "20" y = "3" height = "5" ></ rect > < path d = "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" ></ path > < path d = "m9.5 17 5-5" ></ path > < path d = "m9.5 12 5 5" ></ path > < / > } } pub const LUCIDE_ARCHIVE_X : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;