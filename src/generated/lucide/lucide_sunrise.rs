use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v8" ></ path > < path d = "m4.93 10.93 1.41 1.41" ></ path > < path d = "M2 18h2" ></ path > < path d = "M20 18h2" ></ path > < path d = "m19.07 10.93-1.41 1.41" ></ path > < path d = "M22 22H2" ></ path > < path d = "m8 6 4-4 4 4" ></ path > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < / > } } pub const LucideSunrise : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;