use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v2" ></ path > < path d = "m4.93 4.93 1.41 1.41" ></ path > < path d = "M20 12h2" ></ path > < path d = "m19.07 4.93-1.41 1.41" ></ path > < path d = "M15.947 12.65a4 4 0 0 0-5.925-4.128" ></ path > < path d = "M13 22H7a5 5 0 1 1 4.9-6H13a3 3 0 0 1 0 6Z" ></ path > < / > } } pub const LUCIDE_CLOUD_SUN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;