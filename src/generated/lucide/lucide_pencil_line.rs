use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 20h9" ></ path > < path d = "M16.376 3.622a1 1 0 0 1 3.002 3.002L7.368 18.635a2 2 0 0 1-.855.506l-2.872.838a.5.5 0 0 1-.62-.62l.838-2.872a2 2 0 0 1 .506-.854z" ></ path > < path d = "m15 5 3 3" ></ path > < / > } } pub const LUCIDE_PENCIL_LINE : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24")] } ;