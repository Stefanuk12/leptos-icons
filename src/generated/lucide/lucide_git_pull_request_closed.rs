use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "6" r = "3" cx = "6" ></ circle > < path d = "M6 9v12" ></ path > < path d = "m21 3-6 6" ></ path > < path d = "m21 9-6-6" ></ path > < path d = "M18 11.5V15" ></ path > < circle r = "3" cx = "18" cy = "18" ></ circle > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_CLOSED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;