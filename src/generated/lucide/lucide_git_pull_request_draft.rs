use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "18" r = "3" cx = "18" ></ circle > < circle cx = "6" r = "3" cy = "6" ></ circle > < path d = "M18 6V5" ></ path > < path d = "M18 11v-1" ></ path > < line x2 = "6" x1 = "6" y2 = "21" y1 = "9" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_DRAFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24")] } ;