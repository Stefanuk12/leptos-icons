use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v8" ></ path > < path d = "m16 6-4 4-4-4" ></ path > < rect y = "14" width = "20" rx = "2" x = "2" height = "8" ></ rect > < path d = "M6 18h.01" ></ path > < path d = "M10 18h.01" ></ path > < / > } } pub const LUCIDE_HARD_DRIVE_DOWNLOAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;