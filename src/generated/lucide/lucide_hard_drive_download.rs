use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v8" ></ path > < path d = "m16 6-4 4-4-4" ></ path > < rect x = "2" width = "20" y = "14" rx = "2" height = "8" ></ rect > < path d = "M6 18h.01" ></ path > < path d = "M10 18h.01" ></ path > < / > } } pub const LucideHardDriveDownload : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;