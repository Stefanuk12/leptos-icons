use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 2a26.6 26.6 0 0 1 10 20c.9-6.82 1.5-9.5 4-14" ></ path > < path d = "M16 8c4 0 6-2 6-6-4 0-6 2-6 6" ></ path > < path d = "M17.41 3.6a10 10 0 1 0 3 3" ></ path > < / > } } pub const LUCIDE_VEGAN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24")] } ;