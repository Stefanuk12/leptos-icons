use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m10 10-6.157 6.162a2 2 0 0 0-.5.833l-1.322 4.36a.5.5 0 0 0 .622.624l4.358-1.323a2 2 0 0 0 .83-.5L14 13.982" ></ path > < path d = "m12.829 7.172 4.359-4.346a1 1 0 1 1 3.986 3.986l-4.353 4.353" ></ path > < path d = "m15 5 4 4" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_PENCIL_OFF : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;