use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" x = "9" y = "7" height = "6" ></ rect > < rect height = "14" rx = "2" x = "2" y = "3" width = "20" ></ rect > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < / > } } pub const LucideMonitorStop : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;