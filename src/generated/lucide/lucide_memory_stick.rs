use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 19v-3" ></ path > < path d = "M10 19v-3" ></ path > < path d = "M14 19v-3" ></ path > < path d = "M18 19v-3" ></ path > < path d = "M8 11V9" ></ path > < path d = "M16 11V9" ></ path > < path d = "M12 11V9" ></ path > < path d = "M2 15h20" ></ path > < path d = "M2 7a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.1a2 2 0 0 0 0 3.837V17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-5.1a2 2 0 0 0 0-3.837Z" ></ path > < / > } } pub const LUCIDE_MEMORY_STICK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor")] } ;