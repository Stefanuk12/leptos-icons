use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2" ></ path > < path d = "M2 9h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H2" ></ path > < path d = "M22 17v1a1 1 0 0 1-1 1H10v-9a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v9" ></ path > < circle cx = "8" cy = "19" r = "2" ></ circle > < / > } } pub const LUCIDE_CARAVAN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;