use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" width = "4" y = "9" height = "4" ></ rect > < rect height = "10" x = "10" y = "9" width = "4" ></ rect > < path d = "M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2" ></ path > < circle cy = "19" r = "2" cx = "8" ></ circle > < path d = "M10 19h12v-2" ></ path > < / > } } pub const LUCIDE_CARAVAN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;