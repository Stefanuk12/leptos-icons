use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 22v-5l5-5 5 5-5 5z" ></ path > < path d = "M9.5 14.5 16 8" ></ path > < path d = "m17 2 5 5-.5.5a3.53 3.53 0 0 1-5 0s0 0 0 0a3.53 3.53 0 0 1 0-5L17 2" ></ path > < / > } } pub const LUCIDE_SHOVEL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24")] } ;