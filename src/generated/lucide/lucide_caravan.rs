use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "4" height = "4" y = "9" ></ rect > < rect x = "10" height = "10" width = "4" y = "9" ></ rect > < path d = "M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2" ></ path > < circle r = "2" cy = "19" cx = "8" ></ circle > < path d = "M10 19h12v-2" ></ path > < / > } } pub const LUCIDE_CARAVAN : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none")] } ;