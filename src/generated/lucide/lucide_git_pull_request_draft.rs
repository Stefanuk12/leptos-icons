use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "18" cy = "18" ></ circle > < circle cy = "6" r = "3" cx = "6" ></ circle > < path d = "M18 6V5" ></ path > < path d = "M18 11v-1" ></ path > < line y2 = "21" x2 = "6" y1 = "9" x1 = "6" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_DRAFT : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;