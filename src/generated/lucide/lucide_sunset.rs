use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 10V2" ></ path > < path d = "m4.93 10.93 1.41 1.41" ></ path > < path d = "M2 18h2" ></ path > < path d = "M20 18h2" ></ path > < path d = "m19.07 10.93-1.41 1.41" ></ path > < path d = "M22 22H2" ></ path > < path d = "m16 6-4 4-4-4" ></ path > < path d = "M16 18a4 4 0 0 0-8 0" ></ path > < / > } } pub const LUCIDE_SUNSET : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;