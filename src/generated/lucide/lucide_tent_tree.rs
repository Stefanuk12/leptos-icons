use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "4" r = "2" cx = "4" ></ circle > < path d = "m14 5 3-3 3 3" ></ path > < path d = "m14 10 3-3 3 3" ></ path > < path d = "M17 14V2" ></ path > < path d = "M17 14H7l-5 8h20Z" ></ path > < path d = "M8 14v8" ></ path > < path d = "m9 14 5 8" ></ path > < / > } } pub const LUCIDE_TENT_TREE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2")] } ;