use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v8" ></ path > < path d = "m16 6-4 4-4-4" ></ path > < rect width = "20" x = "2" rx = "2" height = "8" y = "14" ></ rect > < path d = "M6 18h.01" ></ path > < path d = "M10 18h.01" ></ path > < / > } } pub const LUCIDE_HARD_DRIVE_DOWNLOAD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;