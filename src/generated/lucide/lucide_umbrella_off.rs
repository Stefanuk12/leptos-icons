use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v1" ></ path > < path d = "M15.5 21a1.85 1.85 0 0 1-3.5-1v-8H2a10 10 0 0 1 3.428-6.575" ></ path > < path d = "M17.5 12H22A10 10 0 0 0 9.004 3.455" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_UMBRELLA_OFF : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none")] } ;