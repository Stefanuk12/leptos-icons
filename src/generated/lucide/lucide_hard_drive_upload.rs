use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m16 6-4-4-4 4" ></ path > < path d = "M12 2v8" ></ path > < rect width = "20" rx = "2" y = "14" x = "2" height = "8" ></ rect > < path d = "M6 18h.01" ></ path > < path d = "M10 18h.01" ></ path > < / > } } pub const LucideHardDriveUpload : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;