use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 8V4H8" ></ path > < rect y = "8" x = "4" rx = "2" width = "16" height = "12" ></ rect > < path d = "M2 14h2" ></ path > < path d = "M20 14h2" ></ path > < path d = "M15 13v2" ></ path > < path d = "M9 13v2" ></ path > < / > } } pub const LUCIDE_BOT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;