use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m6.5 6.5 11 11" ></ path > < path d = "m21 21-1-1" ></ path > < path d = "m3 3 1 1" ></ path > < path d = "m18 22 4-4" ></ path > < path d = "m2 6 4-4" ></ path > < path d = "m3 10 7-7" ></ path > < path d = "m14 21 7-7" ></ path > < / > } } pub const LucideDumbbell : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;