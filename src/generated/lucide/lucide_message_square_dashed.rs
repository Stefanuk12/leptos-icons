use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 6V5c0-1.1.9-2 2-2h2" ></ path > < path d = "M11 3h3" ></ path > < path d = "M18 3h1c1.1 0 2 .9 2 2" ></ path > < path d = "M21 9v2" ></ path > < path d = "M21 15c0 1.1-.9 2-2 2h-1" ></ path > < path d = "M14 17h-3" ></ path > < path d = "m7 17-4 4v-5" ></ path > < path d = "M3 12v-2" ></ path > < / > } } pub const LUCIDE_MESSAGE_SQUARE_DASHED : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none")] } ;