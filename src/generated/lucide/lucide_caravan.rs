use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "9" height = "4" width = "4" x = "2" ></ rect > < rect x = "10" width = "4" height = "10" y = "9" ></ rect > < path d = "M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2" ></ path > < circle cx = "8" r = "2" cy = "19" ></ circle > < path d = "M10 19h12v-2" ></ path > < / > } } pub const LUCIDE_CARAVAN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;