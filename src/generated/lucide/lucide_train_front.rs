use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 3.1V7a4 4 0 0 0 8 0V3.1" ></ path > < path d = "m9 15-1-1" ></ path > < path d = "m15 15 1-1" ></ path > < path d = "M9 19c-2.8 0-5-2.2-5-5v-4a8 8 0 0 1 16 0v4c0 2.8-2.2 5-5 5Z" ></ path > < path d = "m8 19-2 3" ></ path > < path d = "m16 19 2 3" ></ path > < / > } } pub const LUCIDE_TRAIN_FRONT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;