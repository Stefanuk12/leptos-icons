use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 7.75a.75.75 0 0 1 1.142-.638l3.664 2.249a.75.75 0 0 1 0 1.278l-3.664 2.25a.75.75 0 0 1-1.142-.64z" ></ path > < path d = "M12 17v4" ></ path > < path d = "M8 21h8" ></ path > < rect x = "2" height = "14" width = "20" rx = "2" y = "3" ></ rect > < / > } } pub const LUCIDE_MONITOR_PLAY : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;