use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m21.12 6.4-6.05-4.06a2 2 0 0 0-2.17-.05L2.95 8.41a2 2 0 0 0-.95 1.7v5.82a2 2 0 0 0 .88 1.66l6.05 4.07a2 2 0 0 0 2.17.05l9.95-6.12a2 2 0 0 0 .95-1.7V8.06a2 2 0 0 0-.88-1.66Z" ></ path > < path d = "M10 22v-8L2.25 9.15" ></ path > < path d = "m10 14 11.77-6.87" ></ path > < / > } } pub const LUCIDE_CUBOID : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;