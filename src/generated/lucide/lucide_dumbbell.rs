use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m6.5 6.5 11 11" ></ path > < path d = "m21 21-1-1" ></ path > < path d = "m3 3 1 1" ></ path > < path d = "m18 22 4-4" ></ path > < path d = "m2 6 4-4" ></ path > < path d = "m3 10 7-7" ></ path > < path d = "m14 21 7-7" ></ path > < / > } } pub const LUCIDE_DUMBBELL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;