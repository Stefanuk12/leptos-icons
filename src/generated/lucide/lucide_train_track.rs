use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 17 17 2" ></ path > < path d = "m2 14 8 8" ></ path > < path d = "m5 11 8 8" ></ path > < path d = "m8 8 8 8" ></ path > < path d = "m11 5 8 8" ></ path > < path d = "m14 2 8 8" ></ path > < path d = "M7 22 22 7" ></ path > < / > } } pub const LUCIDE_TRAIN_TRACK : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;