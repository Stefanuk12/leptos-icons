use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 6-4-4-4 4" ></ path > < path d = "M12 2v8" ></ path > < rect x = "2" height = "8" rx = "2" width = "20" y = "14" ></ rect > < path d = "M6 18h.01" ></ path > < path d = "M10 18h.01" ></ path > < / > } } pub const LUCIDE_HARD_DRIVE_UPLOAD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;