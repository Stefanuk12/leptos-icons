use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "9" width = "4" x = "2" height = "4" ></ rect > < rect height = "10" width = "4" y = "9" x = "10" ></ rect > < path d = "M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2" ></ path > < circle cy = "19" cx = "8" r = "2" ></ circle > < path d = "M10 19h12v-2" ></ path > < / > } } pub const LUCIDE_CARAVAN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;