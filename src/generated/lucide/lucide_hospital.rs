use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6v4" ></ path > < path d = "M14 14h-4" ></ path > < path d = "M14 18h-4" ></ path > < path d = "M14 8h-4" ></ path > < path d = "M18 12h2a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-9a2 2 0 0 1 2-2h2" ></ path > < path d = "M18 22V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v18" ></ path > < / > } } pub const LUCIDE_HOSPITAL : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;