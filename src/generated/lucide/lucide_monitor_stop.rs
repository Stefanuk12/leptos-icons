use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "6" height = "6" y = "7" x = "9" ></ rect > < rect width = "20" x = "2" height = "14" y = "3" rx = "2" ></ rect > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < / > } } pub const LucideMonitorStop : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;