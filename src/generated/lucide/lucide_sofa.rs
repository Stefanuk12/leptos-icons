use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3" ></ path > < path d = "M2 11v5a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v2H6v-2a2 2 0 0 0-4 0Z" ></ path > < path d = "M4 18v2" ></ path > < path d = "M20 18v2" ></ path > < path d = "M12 4v9" ></ path > < / > } } pub const LUCIDE_SOFA : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;