use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "20" height = "5" x = "2" y = "3" rx = "1" ></ rect > < path d = "M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" ></ path > < path d = "m9.5 17 5-5" ></ path > < path d = "m9.5 12 5 5" ></ path > < / > } } pub const LUCIDE_ARCHIVE_X : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;