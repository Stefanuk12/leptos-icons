use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 8V4H8" ></ path > < rect x = "4" width = "16" y = "8" height = "12" rx = "2" ></ rect > < path d = "M2 14h2" ></ path > < path d = "M20 14h2" ></ path > < path d = "M15 13v2" ></ path > < path d = "M9 13v2" ></ path > < / > } } pub const LUCIDE_BOT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;