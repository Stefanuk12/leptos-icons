use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" cy = "18" r = "3" ></ circle > < circle cx = "6" r = "3" cy = "6" ></ circle > < path d = "M18 6V5" ></ path > < path d = "M18 11v-1" ></ path > < line y1 = "9" x2 = "6" y2 = "21" x1 = "6" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_DRAFT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor")] } ;