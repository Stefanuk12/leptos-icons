use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 2h4" ></ path > < path d = "M12 14v-4" ></ path > < path d = "M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6" ></ path > < path d = "M9 17H4v5" ></ path > < / > } } pub const LUCIDE_TIMER_RESET : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;