use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 8V4H8" ></ path > < rect width = "16" rx = "2" height = "12" x = "4" y = "8" ></ rect > < path d = "M2 14h2" ></ path > < path d = "M20 14h2" ></ path > < path d = "M15 13v2" ></ path > < path d = "M9 13v2" ></ path > < / > } } pub const LUCIDE_BOT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;