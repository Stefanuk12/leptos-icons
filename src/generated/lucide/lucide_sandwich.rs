use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 11v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-3" ></ path > < path d = "M12 19H4a1 1 0 0 1-1-1v-2a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3.83" ></ path > < path d = "m3 11 7.77-6.04a2 2 0 0 1 2.46 0L21 11H3Z" ></ path > < path d = "M12.97 19.77 7 15h12.5l-3.75 4.5a2 2 0 0 1-2.78.27Z" ></ path > < / > } } pub const LUCIDE_SANDWICH : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24")] } ;